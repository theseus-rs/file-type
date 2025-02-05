use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_122676986: FileFormat = FileFormat {
    id: 122_676_986,
    source_type: SourceType::Wikidata,
    name: "CMX Corel Clipart",
    extensions: &["cmx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
