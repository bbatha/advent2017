pipeline {
  stages {
    agent docker {
       image 'rust:1.26'
       label 'docker'
    }

    stage('Initialize') {
      steps {
        sh '''
          echo "PATH = ${PATH}"
        '''
        sh 'cargo install --git https://github.com/DSRCorporation/cargo-test-xunit'
        stash name: 'test-xunit', includes: '~/.cargo/bin/cargo-test-xunit'
      }
    }

    stage('Test') {
      steps {
        unstash name: 'test-xunit'
        sh './cargo-test-xunit'
      }
      post {
         always {
            junit 'test-results.xml'
         }
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
