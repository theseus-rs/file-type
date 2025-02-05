use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27823992: FileFormat = FileFormat {
    id: 27_823_992,
    source_type: SourceType::Wikidata,
    name: "Maptech BSB documentation file, version 3.0",
    extensions: &["bsb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
