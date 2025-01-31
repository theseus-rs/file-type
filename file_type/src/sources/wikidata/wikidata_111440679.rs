use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111440679: FileFormat = FileFormat {
    id: 111_440_679,
    puid: "wikidata/111440679",
    name: "Perl POD File",
    extensions: &["pod"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
