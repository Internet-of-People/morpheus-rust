use super::*;

/// Multicipher [`Signature`]
///
/// [`Signature`]: ../trait.AsymmetricCrypto.html#associatedtype.Signature
#[derive(Clone, Eq, PartialEq)]
pub enum MSignature {
    /// The signature tagged with this variant belongs to the [`ed25519`] module
    ///
    /// [`ed25519`]: ../ed25519/index.html
    Ed25519(EdSignature),
    /// The signature tagged with this variant belongs to the [`secp256k1`] module
    ///
    /// [`secp256k1`]: ../secp256k1/index.html
    Secp256k1(SecpSignature),
}

impl MSignature {
    /// All multicipher signatures start with this prefix
    pub const PREFIX: char = 's';

    /// The ciphersuite that this signature belongs to
    pub fn suite(&self) -> CipherSuite {
        match self {
            Self::Ed25519(_) => CipherSuite::Ed25519,
            Self::Secp256k1(_) => CipherSuite::Secp256k1,
        }
    }

    /// Even the binary representation of a multicipher signature is readable with this.
    // TODO Should we really keep it like this?
    pub fn to_bytes(&self) -> Vec<u8> {
        String::from(self).as_bytes().to_vec()
    }

    /// Even the binary representation of a multicipher signature is readable with this.
    // TODO Should we really keep it like this?
    pub fn from_bytes<B: AsRef<[u8]>>(bytes: B) -> Result<Self> {
        let string = String::from_utf8(bytes.as_ref().to_owned())?;
        string.parse()
    }

    fn to_inner_bytes(&self) -> Vec<u8> {
        match self {
            Self::Ed25519(edsig) => edsig.to_bytes(),
            Self::Secp256k1(secpsig) => secpsig.to_bytes(),
        }
    }

    fn from_inner_bytes<B: AsRef<[u8]>>(suite: char, inner_bytes: B) -> Result<Self> {
        match CipherSuite::from_char(suite)? {
            CipherSuite::Ed25519 => Ok(Self::Ed25519(EdSignature::from_bytes(inner_bytes)?)),
            CipherSuite::Secp256k1 => Ok(Self::Secp256k1(SecpSignature::from_bytes(inner_bytes)?)),
        }
    }
}

impl Serialize for MSignature {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let erased = ErasedBytes { suite: self.suite().as_byte(), value: self.to_inner_bytes() };
        erased.serialize(serializer)
    }
}

fn deser(erased: ErasedBytes) -> Result<MSignature> {
    MSignature::from_inner_bytes(erased.suite as char, &erased.value)
}

impl<'de> Deserialize<'de> for MSignature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        ErasedBytes::deserialize(deserializer)
            .and_then(|b| deser(b).map_err(|e| serde::de::Error::custom(e.to_string())))
    }
}

impl From<&MSignature> for String {
    fn from(src: &MSignature) -> Self {
        let mut output = multibase::encode(multibase::Base::Base58Btc, src.to_inner_bytes());
        output.insert(0, src.suite().as_char());
        output.insert(0, MSignature::PREFIX);
        output
    }
}

impl From<MSignature> for String {
    fn from(src: MSignature) -> Self {
        (&src).into()
    }
}

impl fmt::Display for MSignature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&String::from(self))
    }
}

impl fmt::Debug for MSignature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl std::str::FromStr for MSignature {
    type Err = anyhow::Error;
    fn from_str(src: &str) -> Result<Self> {
        let mut chars = src.chars();
        ensure!(
            chars.next() == Some(Self::PREFIX),
            "Signatures must start with '{}'",
            Self::PREFIX
        );
        if let Some(suite) = chars.next() {
            let (_base, binary) = multibase::decode(chars.as_str())?;
            Self::from_inner_bytes(suite, &binary)
        } else {
            Err(anyhow!("No crypto suite found"))
        }
    }
}

impl From<EdSignature> for MSignature {
    fn from(src: EdSignature) -> Self {
        Self::Ed25519(src)
    }
}

impl From<SecpSignature> for MSignature {
    fn from(src: SecpSignature) -> Self {
        Self::Secp256k1(src)
    }
}

#[cfg(test)]
mod test {
    mod parse_signature {
        use crate::ed25519::EdSignature;
        use crate::multicipher::{CipherSuite, MSignature};

        #[allow(dead_code)]
        fn case(input: &str, sig_hex: &str) {
            let sig_bytes = hex::decode(sig_hex.replace(' ', "")).unwrap();
            let sig1 = EdSignature::from_bytes(&sig_bytes).unwrap();
            let erased_sig1 = MSignature::from(sig1);
            assert_eq!(erased_sig1.to_string(), input);

            let erased_sig2 = input.parse::<MSignature>().unwrap();
            assert_eq!(erased_sig2, erased_sig1);
        }

        #[test]
        fn test_1() {
            case(
                "sezAhoNep8B9HTRCAYaJFPL1hNgqxfjM72UD4B75s258aF6pPCtDf5trXm7mppZVzT6ynpC3jyH6h3Li7r9Rw4yjeG2",
                "01e5564300c360ac729086e2cc806e828a84877f1eb8e5d974d873e06522490155 \
                 5fb8821590a33bacc61e39701cf9b46bd25bf5f0595bbe24655141438e7a100b",
            );
        }

        #[test]
        fn test_2() {
            case(
                "sez93tR11WBTZjw25Ht3CgaiSaC5rb3GnkAcaAUznjomtVj6Ac4rzQ4df9Fvy1uitGe8ZSBiG4Q5ukaVo5sjMpAwkxX",
                "0192a009a9f0d4cab8720e820b5f642540a2b27b5416503f8fb3762223ebdb69da \
                 085ac1e43e15996e458f3613d0f11d8c387b2eaeb4302aeeb00d291612bb0c00",
            );
        }

        #[test]
        fn test_3() {
            case(
                "sez86ALkZRspsufndsFkaT3GS5m4FHxUTGhBPRpdqqgfdgCMPWzDvxHjVAZXQNVPH8vHohuRkLtEWtT9guyscG2WsZB",
                "016291d657deec24024827e69c3abe01a30ce548a284743a445e3680d7db5ac3ac \
                 18ff9b538d16f290ae67f760984dc6594a7c15e9716ed28dc027beceea1ec40a"
            );
        }

        #[test]
        fn ed_suite() {
            let sig = "sezAhoNep8B9HTRCAYaJFPL1hNgqxfjM72UD4B75s258aF6pPCtDf5trXm7mppZVzT6ynpC3jyH6h3Li7r9Rw4yjeG2".parse::<MSignature>().unwrap();
            assert_eq!(sig.suite(), CipherSuite::Ed25519);
            assert!(matches!(&sig, MSignature::Ed25519(_)));
            assert!(!matches!(&sig, MSignature::Secp256k1(_)));
        }

        #[test]
        fn secp_suite() {
            let sig = "ssz8XFYUjuSro2dzq4mkMMCMJkPH2SEEc6CVJ9VCG9AUXVvRP9QHKY78BnSvpb9zyz5yZf8Pzcq82DzZwLC7xSeGNgq".parse::<MSignature>().unwrap();
            assert_eq!(sig.suite(), CipherSuite::Secp256k1);
            assert!(!matches!(&sig, MSignature::Ed25519(_)));
            assert!(matches!(&sig, MSignature::Secp256k1(_)));
        }

        #[test]
        #[should_panic(expected = "Unknown crypto suite 'g'")]
        fn invalid_suite() {
            let _sig =
                "sgzAhoNep8B9HTRCAYaJFPL1hNgqxfjM72UD4B75s258aF6pPCtDf5trXm7mppZVzT6ynpC3jyH6h3Li7r9Rw4yjeG2".parse::<MSignature>().unwrap();
        }

        #[test]
        #[should_panic(expected = "No crypto suite found")]
        fn missing_suite() {
            let _sig = "s".parse::<MSignature>().unwrap();
        }

        #[test]
        #[should_panic(expected = "Signatures must start with 's'")]
        fn invalid_type() {
            let _sig = "FezAhoNep8B9HTRCAYaJFPL1hNgqxfjM72UD4B75s258aF6pPCtDf5trXm7mppZVzT6ynpC3jyH6h3Li7r9Rw4yjeG2".parse::<MSignature>().unwrap();
        }

        #[test]
        #[should_panic(expected = "Signatures must start with 's'")]
        fn empty() {
            let _sig = "".parse::<MSignature>().unwrap();
        }
    }

    mod serde_public_key {
        use crate::multicipher::MSignature;

        #[test]
        fn messagepack_serialization() {
            let sig_str = "sezAhoNep8B9HTRCAYaJFPL1hNgqxfjM72UD4B75s258aF6pPCtDf5trXm7mppZVzT6ynpC3jyH6h3Li7r9Rw4yjeG2";
            let sig = sig_str.parse::<MSignature>().unwrap();
            let sig_bin = rmp_serde::to_vec(&sig).unwrap();

            assert_eq!(
                sig_bin,
                vec![
                    146, 101, 196, 65, 1, 229, 86, 67, 0, 195, 96, 172, 114, 144, 134, 226, 204,
                    128, 110, 130, 138, 132, 135, 127, 30, 184, 229, 217, 116, 216, 115, 224, 101,
                    34, 73, 1, 85, 95, 184, 130, 21, 144, 163, 59, 172, 198, 30, 57, 112, 28, 249,
                    180, 107, 210, 91, 245, 240, 89, 91, 190, 36, 101, 81, 65, 67, 142, 122, 16,
                    11
                ]
            );

            let sig_deser: MSignature = rmp_serde::from_slice(&sig_bin).unwrap();
            let sig_tostr = sig_deser.to_string();
            assert_eq!(sig, sig_deser);
            assert_eq!(sig_str, sig_tostr);
        }

        #[test]
        fn json_serialization() {
            let sig_str = "sezAhoNep8B9HTRCAYaJFPL1hNgqxfjM72UD4B75s258aF6pPCtDf5trXm7mppZVzT6ynpC3jyH6h3Li7r9Rw4yjeG2";
            let sig = sig_str.parse::<MSignature>().unwrap();
            let sig_bin = serde_json::to_vec(&sig).unwrap();

            assert_eq!(sig_bin, br#"{"s":101,"v":[1,229,86,67,0,195,96,172,114,144,134,226,204,128,110,130,138,132,135,127,30,184,229,217,116,216,115,224,101,34,73,1,85,95,184,130,21,144,163,59,172,198,30,57,112,28,249,180,107,210,91,245,240,89,91,190,36,101,81,65,67,142,122,16,11]}"#.to_vec());

            let sig_deser: MSignature = serde_json::from_slice(&sig_bin).unwrap();
            let sig_tostr = sig_deser.to_string();
            assert_eq!(sig, sig_deser);
            assert_eq!(sig_str, sig_tostr);
        }
    }

    /// Test vectors based on https://tools.ietf.org/html/rfc8032#page-24
    mod sign_verify {
        use crate::{ed25519::EdPrivateKey, multicipher::MPrivateKey};
        use crate::{PrivateKey, PublicKey};

        fn test(sk_hex: &str, mpk_str: &str, msg_hex: &str, msig_str: &str) {
            let sk_bytes = hex::decode(sk_hex).unwrap();
            let ed_sk = EdPrivateKey::from_bytes(sk_bytes.as_slice()).unwrap();
            let msk = MPrivateKey::from(ed_sk);

            let mpk = msk.public_key();
            assert_eq!(mpk.to_string(), mpk_str.to_owned());

            let msg = hex::decode(msg_hex.replace(' ', "")).unwrap();
            let msig = msk.sign(msg.as_slice());
            assert_eq!(msig.to_string(), msig_str.to_owned());

            assert!(mpk.verify(msg, &msig));
        }

        #[test]
        fn char_0() {
            test(
                "9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60",
                "pezFVen3X669xLzsi6N2V91DoiyzHzg1uAgqiT8jZ9nS96Z",
                "",
                "sezAhoNep8B9HTRCAYaJFPL1hNgqxfjM72UD4B75s258aF6pPCtDf5trXm7mppZVzT6ynpC3jyH6h3Li7r9Rw4yjeG2",
            );
        }

        #[test]
        fn char_1() {
            test(
                "4ccd089b28ff96da9db6c346ec114e0f5b8a319f35aba624da8cf6ed4fb8a6fb",
                "pez586Z7H2vpX9qNhN2T4e9Utugie3ogjbxzGaMtM3E6HR5",
                "72",
                "sez93tR11WBTZjw25Ht3CgaiSaC5rb3GnkAcaAUznjomtVj6Ac4rzQ4df9Fvy1uitGe8ZSBiG4Q5ukaVo5sjMpAwkxX",
            );
        }

        #[test]
        fn char_2() {
            test(
                "c5aa8df43f9f837bedb7442f31dcb7b166d38535076f094b85ce3a2e0b4458f7",
                "pezHyx62wPQGyvXCoihZq1BrbUjBRh2LuNxWiiqMkfAuSZr",
                "af82",
                "sez86ALkZRspsufndsFkaT3GS5m4FHxUTGhBPRpdqqgfdgCMPWzDvxHjVAZXQNVPH8vHohuRkLtEWtT9guyscG2WsZB",
            );
        }
    }
}
