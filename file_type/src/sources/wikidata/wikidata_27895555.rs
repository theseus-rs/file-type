use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27895555: FileFormat = FileFormat {
    id: 27_895_555,
    source_type: SourceType::Wikidata,
    name: "ADX, version 3",
    extensions: &["adx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
