use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_104600905: FileFormat = FileFormat {
    id: 104_600_905,
    source_type: SourceType::Wikidata,
    name: "KDBX",
    extensions: &["kdbx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
