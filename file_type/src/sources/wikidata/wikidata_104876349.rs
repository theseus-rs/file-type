use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_104876349: FileFormat = FileFormat {
    id: 104_876_349,
    source_type: SourceType::Wikidata,
    name: "JCAMP-DX",
    extensions: &["dx", "jcm", "jdx"],
    media_types: &["chemical/x-jcamp-dx"],
    internal_signatures: &[],
    related_formats: &[],
};
