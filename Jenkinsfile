@Library('matrix') _

pipeline {
  agent none

  stages {
    stage('Initialize') {
      agent {
        docker {
          image 'rust:1.26-slim'
          label 'docker'
        }
      }

      steps {
        sh '''
          echo "PATH = ${PATH}"
        '''
        //sh 'cargo install --git https://github.com/DSRCorporation/cargo-test-xunit --root target/release/.'
        //stash name: 'test-xunit', includes: 'target/release/bin/cargo-test-xunit'
      }
    }

    stage('Test') {
      agent { label 'docker' };

      steps {
        matrix(['rust:1.26-slim-stretch', 'rust:1.26-slim'], {
          //unstash name: 'test-xunit'
          //sh './target/release/bin/cargo-test-xunit'
          sh 'cargo test'
        })
      }
    }

    stage('Build') {
      agent {
        docker {
          image 'rust:1.26-slim'
          label 'docker'
        }
      }

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
