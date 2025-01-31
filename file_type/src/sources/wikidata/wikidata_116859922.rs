use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_116859922: FileFormat = FileFormat {
    id: 116_859_922,
    puid: "wikidata/116859922",
    name: "Test File",
    extensions: &["tst"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
