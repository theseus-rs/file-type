use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967179: FileFormat = FileFormat {
    id: 27_967_179,
    source_type: SourceType::Wikidata,
    name: "Farandole Form 2.0",
    extensions: &["f2r"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
