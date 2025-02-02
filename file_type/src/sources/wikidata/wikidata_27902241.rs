use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27902241: FileFormat = FileFormat {
    id: 27_902_241,
    source_type: SourceType::Wikidata,
    name: "Amateur Data Interchange Format, ADX variant, version 3.0.4",
    extensions: &["adx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
