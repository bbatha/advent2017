@Library('matrix') _

pipeline {
  agent {
    docker {
      image 'rust:1.26'
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
        stash name: 'test-xunit', includes: 'target/release/cargo-test-xunit'
      }
    }

    stage('Test') {
      matrix ['1.24', '1.25', '1.26'], {
        unstash name: 'test-xunit'
        sh './cargo-test-xunit'
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
