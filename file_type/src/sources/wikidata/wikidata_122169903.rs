use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122169903: FileFormat = FileFormat {
    id: 122_169_903,
    source_type: SourceType::Wikidata,
    name: "PPA and PWDUMP Password Hashes",
    extensions: &["hdc"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
