use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_131010842: FileFormat = FileFormat {
    id: 131_010_842,
    puid: "wikidata/131010842",
    name: "Smithy file format",
    extensions: &["smithy"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
