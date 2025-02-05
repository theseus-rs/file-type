use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_104600905: FileFormat = FileFormat {
    id: 104_600_905,
    source_type: SourceType::Wikidata,
    name: "KDBX",
    extensions: &["kdbx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
