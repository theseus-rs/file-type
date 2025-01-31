use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111578627: FileFormat = FileFormat {
    id: 111_578_627,
    puid: "wikidata/111578627",
    name: "Z Print Build File",
    extensions: &["zbd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
