use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111009526: FileFormat = FileFormat {
    id: 111_009_526,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Certificate File format",
    extensions: &["cer"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
