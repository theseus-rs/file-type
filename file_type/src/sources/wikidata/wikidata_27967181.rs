use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967181: FileFormat = FileFormat {
    id: 27_967_181,
    source_type: SourceType::Wikidata,
    name: "Farandole Composer pattern",
    extensions: &["fpt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
