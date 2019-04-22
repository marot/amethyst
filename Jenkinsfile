pipeline {
    agent none
    stages {
	stage("Pull new images") {
	    agent {
		label 'docker'
	    }
	    steps {
		sh 'docker pull amethystrs/builder-linux:stable'
		sh 'docker pull amethystrs/builder-linux:nightly'
	    }
	}
        stage('Cargo Fmt') {
            environment {
                RUSTFLAGS = "-D warnings"
            }
            agent {
		docker {
		    image 'amethystrs/builder-linux:stable'
		    label 'docker'
		} 
            }
            steps {
                echo 'Checking formatting...'
                sh 'cargo fmt -- --check'
            }
        }
        stage('Cargo Check') {
            parallel {
                stage("stable") {
                    environment {
                    RUSTFLAGS = "-D warnings"
                    }
                    agent {
                    docker {
                        image 'amethystrs/builder-linux:stable'
                        label 'docker'
                    } 
                    }
                    steps {
                    echo 'Running Cargo check...'
                    sh 'cargo check --all --all-targets --features sdl_controller,json,saveload'
                    }
                }
                stage("nightly") {
                    environment {
                    RUSTFLAGS = "-D warnings"
                    }
                    agent {
                    docker {
                        image 'amethystrs/builder-linux:nightly'
                        label 'docker'
                    } 
                    }
                    steps {
                    echo 'Running Cargo check...'
                    sh 'cargo check --all --all-targets --features nightly'
                    }
                }
            }
        }
        stage('Run Tests') {
            parallel {
                stage("Test on Windows") {                    
                    environment {
                        CARGO_HOME = 'C:\\Users\\root\\.cargo'
                        RUSTUP_HOME = 'C:\\Users\\root\\.rustup'
                    }
                    agent { 
                        label 'windows' 
                    }
                    steps {
                        echo 'Beginning tests...'
                        bat 'C:\\Users\\root\\.cargo\\bin\\cargo test --all'
                        echo 'Tests done!'
                    }
                }
                stage("Test on Linux") {
                    agent {
			            docker {
			                image 'amethystrs/builder-linux:stable'
			                label 'docker'
			            }
                    }
                    steps {
                        echo 'Beginning tests...'
                        sh 'cargo test --all'
                        echo 'Tests done!'
                    }
                }
                stage('Calculate Coverage') {
                    environment {
                        CARGO_HOME = '/home/jenkins/.cargo'
                        RUSTUP_HOME = '/home/jenkins/.rustup'
                        RUSTFLAGS = "-D warnings"
                    }
                    agent {
			            docker {
			                image 'amethystrs/builder-linux:stable'
			                label 'docker'
			            }
                    }
                    steps {
                        withCredentials([string(credentialsId: 'codecov_token', variable: 'CODECOV_TOKEN')]) {
                            echo 'Building to calculate coverage'
                            sh 'cargo test --all'
                            echo 'Calculating code coverage...'
                            sh 'for file in target/debug/amethyst_*[^\\.d]; do mkdir -p \"target/cov/$(basename $file)\"; kcov --exclude-pattern=/.cargo,/usr/lib --verify \"target/cov/$(basename $file)\" \"$file\" || true; done'
                            echo "Uploading coverage..."
                            sh "curl -s https://codecov.io/bash | bash -s - -t $CODECOV_TOKEN"
                            echo "Uploaded code coverage!"
                        }
                    }
                }
                // macOS is commented out due to needing to upgrade the OS, but MacStadium did not do the original install with APFS so we cannot upgrade easily
                // stage("Test on macOS") {
                //     environment {
                //         CARGO_HOME = '/Users/jenkins/.cargo'
                //         RUSTUP_HOME = '/Users/jenkins/.rustup'
                //     }
                //     agent {
                //         label 'mac'
                //     }
                //     steps {
                //         echo 'Beginning tests...'
                //         sh '/Users/jenkins/.cargo/bin/cargo test --all'
                //         echo 'Tests done!'
                //     }
                // }
            }
        }

    }
}
