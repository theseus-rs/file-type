use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_2679202: FileFormat = FileFormat {
    id: 2_679_202,
    source_type: SourceType::Wikidata,
    name: "nds",
    extensions: &["nds"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
