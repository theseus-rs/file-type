use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111355515: FileFormat = FileFormat {
    id: 111_355_515,
    puid: "wikidata/111355515",
    name: "Talking Technology Incorporated file",
    extensions: &["vox"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
