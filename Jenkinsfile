pipeline {
  agent 'rust'

  stages {
    stage('Initialize') {
      steps {
        sh '''
          echo "PATH = ${PATH}"
        '''
      }
    }

    stage('Test') {
      steps {
        sh 'cargo test'
      }
    }

    stage('Build') {
      steps {
        sh 'cargo build --release'
      }
    }

    post {
      success {
        archiveArtifacts artifacts: 'target/release/day*', fingerprint: true
      }
    }
  }
}
