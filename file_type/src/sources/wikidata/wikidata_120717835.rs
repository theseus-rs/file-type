use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_120717835: FileFormat = FileFormat {
    id: 120_717_835,
    source_type: SourceType::Wikidata,
    name: "DeductionPro 2006 Data File",
    extensions: &["d06"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
