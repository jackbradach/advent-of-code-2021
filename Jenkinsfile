pipeline {
    agent {
        docker {
            image 'rust:buster'
        }
    }

    stages {
        stage('Checkout') {
            steps {
                checkout scm
            }
        }
        stage('Build') {
            steps {
                sh "cargo build"
            }
        }
        
        stage('Test') {
            steps {
                sh "cargo test"
            }
        }
        stage('Clippy') {
            steps {
                sh "cargo +nightly clippy --all"
            }
        }
        stage('Rustfmt') {
            steps {
                // The build will fail if rustfmt thinks any changes are
                // required.
                sh "cargo +nightly fmt --all -- --write-mode diff"
            }
        }
    }
}
