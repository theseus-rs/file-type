use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111009526: FileFormat = FileFormat {
    id: 111_009_526,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Certificate File format",
    extensions: &["cer"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
