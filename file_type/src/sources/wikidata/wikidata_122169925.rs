use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122169925: FileFormat = FileFormat {
    id: 122_169_925,
    source_type: SourceType::Wikidata,
    name: "GPU PWDUMP Password Hashes (NTLM)",
    extensions: &["pwdump"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
