use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_104876349: FileFormat = FileFormat {
    id: 104_876_349,
    source_type: SourceType::Wikidata,
    name: "JCAMP-DX",
    extensions: &["dx", "jcm", "jdx"],
    media_types: &["chemical/x-jcamp-dx"],
    signatures: &[],
    related_formats: &[],
};
