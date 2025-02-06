use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_34743161: FileFormat = FileFormat {
    id: 34_743_161,
    source_type: SourceType::Wikidata,
    name: "Softdisk Publishing UDF",
    extensions: &["udf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
