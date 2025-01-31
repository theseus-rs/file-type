use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111271738: FileFormat = FileFormat {
    id: 111_271_738,
    puid: "wikidata/111271738",
    name: "Delusion/XTracker sample format",
    extensions: &["dsf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
