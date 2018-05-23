@Library('matrix') _

pipeline {
  agent {
    docker {
      image 'rust:1.26-slim'
      label 'docker'
    }
  }

  stages {
    stage('Initialize') {
      steps {
        sh '''
          echo "PATH = ${PATH}"
        '''
        sh 'cargo install --git https://github.com/DSRCorporation/cargo-test-xunit --root target/release/.'
        stash name: 'test-xunit', includes: 'target/release/bin/cargo-test-xunit'
      }
    }

    stage('Test') {
      steps {
        matrix(['1.24-slim', '1.25-slim', '1.26-slim'], {
          unstash name: 'test-xunit'
          sh './cargo-test-xunit'
        })
      }
    }

    stage('Build') {
      steps {
        sh 'cargo build --release'
      }

      post {
        success {
          archiveArtifacts artifacts: 'target/release/day*', excludes: 'target/release/day*.d', fingerprint: true
        }
      }
    }
  }
}
