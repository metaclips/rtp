#[cfg(test)]
mod tests {
    use crate::codecs::g711::*;

    #[test]
    fn test_g711_payload() {
        let pck = G711Payloader;

        const TEST_LEN: usize = 10000;
        const TEST_MTU: u16 = 1500;

        // generate random 8-bit g722 samples
        let samples: Vec<u8> = (0..TEST_LEN).map(|_| rand::random::<u8>()).collect();

        // make a copy, for payloader input
        let mut samples_in = vec![0; TEST_LEN];
        samples_in.clone_from_slice(&samples);

        // split our samples into payloads
        let payloads = pck.payload(TEST_MTU, bytes::Bytes::from_static(&[0; TEST_LEN]));

        let outcnt = ((TEST_LEN as f64) / (TEST_MTU as f64)).ceil() as usize;
        assert_eq!(
            outcnt,
            payloads.len(),
            "Generated {} payloads instead of {}",
            payloads.len(),
            outcnt
        );
        assert_eq!(&samples, &samples_in, "Modified input samples");

        let samples_out = payloads.concat();
        assert_eq!(&samples_out, &samples_in, "Output samples don't match");

        // Positive MTU, empty payload
        let result = pck.payload(1, bytes::Bytes::new());
        assert!(result.is_empty(), "Generated payload should be empty");

        // 0 MTU, small payload
        let result = pck.payload(0, bytes::Bytes::from_static(&[0x90, 0x90, 0x90]));
        assert!(result.is_empty(), "Generated payload should be empty");

        // Positive MTU, small payload
        let result = pck.payload(10, bytes::Bytes::from_static(&[0x90, 0x90, 0x90]));
        assert_eq!(result.len(), 1, "Generated payload should be the 1");
    }
}
