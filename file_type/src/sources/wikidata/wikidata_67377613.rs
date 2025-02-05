use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_67377613: FileFormat = FileFormat {
    id: 67_377_613,
    source_type: SourceType::Wikidata,
    name: "CaseWare 2005 Compressed file",
    extensions: &["ac_"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
