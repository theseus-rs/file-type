use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_122169925: FileFormat = FileFormat {
    id: 122_169_925,
    source_type: SourceType::Wikidata,
    name: "GPU PWDUMP Password Hashes (NTLM)",
    extensions: &["pwdump"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
