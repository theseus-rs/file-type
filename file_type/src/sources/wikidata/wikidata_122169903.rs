use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122169903: FileFormat = FileFormat {
    id: 122_169_903,
    source_type: SourceType::Wikidata,
    name: "PPA and PWDUMP Password Hashes",
    extensions: &["hdc"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
