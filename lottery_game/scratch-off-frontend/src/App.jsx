import { useState } from 'react';
import { ethers } from 'ethers';

const contractAddress = '0xd35eae619f2514f454fc3b832c3aa85e884b7c15'; // Your deployed contract address

const abi = [
  "function scratch() public returns (bool)",
  "function checkResult(address player) public view returns (bool)"
];

function App() {
  const [account, setAccount] = useState(null);
  const [message, setMessage] = useState("");
  const [loading, setLoading] = useState(false);

  async function connectWallet() {
    if (window.ethereum) {
      try {
        const accounts = await window.ethereum.request({ method: 'eth_requestAccounts' });
        setAccount(accounts[0]);
      } catch (error) {
        console.error("Connection Error:", error);
      }
    } else {
      alert('Please install MetaMask!');
    }
  }

  async function scratch() {
    if (!account) {
      alert('Please connect your wallet first!');
      return;
    }

    try {
      setLoading(true);
      const provider = new ethers.BrowserProvider(window.ethereum);
      const signer = await provider.getSigner();
      const contract = new ethers.Contract(contractAddress, abi, signer);

      const tx = await contract.scratch();
      await tx.wait();

      const result = await contract.checkResult(account);
      setMessage(result ? "🏆 You WON! 🎉" : "💩 You Lost! 😢");
    } catch (error) {
      console.error("Scratch Error:", error);
      setMessage("Something went wrong! Please try again.");
    } finally {
      setLoading(false);
    }
  }

  return (
    <div style={{
      width: '100%',
      minHeight: '100vh',
      display: 'flex',
      flexDirection: 'column',
      justifyContent: 'center',
      alignItems: 'center',
      background: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
      color: 'white',
      fontFamily: 'Arial, sans-serif',
      textAlign: 'center',
      padding: '2rem',
      boxSizing: 'border-box'
    }}>
      <h1 style={{ fontSize: '3rem', marginBottom: '2rem' }}>Scratch-Off Game 🎯</h1>

      {!account ? (
        <button
          onClick={connectWallet}
          style={{
            padding: '1rem 2rem',
            fontSize: '1.5rem',
            background: 'linear-gradient(90deg, #4facfe 0%, #00f2fe 100%)',
            color: 'white',
            border: 'none',
            borderRadius: '10px',
            cursor: 'pointer',
            transition: 'transform 0.2s'
          }}
          onMouseOver={(e) => e.target.style.transform = 'scale(1.05)'}
          onMouseOut={(e) => e.target.style.transform = 'scale(1)'}
        >
          🔗 Connect Wallet
        </button>
      ) : loading ? (
        <>
          <div style={{ fontSize: '2rem', marginTop: '2rem' }}>⏳ Waiting for transaction...</div>
          <div className="spinner" style={{
            marginTop: '1.5rem',
            width: '50px',
            height: '50px',
            border: '5px solid #f3f3f3',
            borderTop: '5px solid #3498db',
            borderRadius: '50%',
            animation: 'spin 1s linear infinite'
          }} />
          <style>
            {`
              @keyframes spin {
                0% { transform: rotate(0deg); }
                100% { transform: rotate(360deg); }
              }
            `}
          </style>
        </>
      ) : (
        <>
          <div style={{ fontSize: '1.2rem', marginBottom: '1rem' }}>
            Connected: {account.slice(0, 6)}...{account.slice(-4)}
          </div>

          <button
            onClick={scratch}
            style={{
              padding: '1rem 2rem',
              fontSize: '1.5rem',
              background: 'linear-gradient(90deg, #43e97b 0%, #38f9d7 100%)',
              color: 'white',
              border: 'none',
              borderRadius: '10px',
              cursor: 'pointer',
              marginBottom: '2rem',
              transition: 'transform 0.2s'
            }}
            onMouseOver={(e) => e.target.style.transform = 'scale(1.05)'}
            onMouseOut={(e) => e.target.style.transform = 'scale(1)'}
          >
            🎯 Scratch!
          </button>

          {message && (
            <div style={{
              fontSize: '2.5rem',
              fontWeight: 'bold',
              marginTop: '2rem',
              animation: 'fadeIn 1s ease-in-out'
            }}>
              {message}
            </div>
          )}
        </>
      )}
    </div>
  );
}

export default App;