use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_29209269: FileFormat = FileFormat {
    id: 29_209_269,
    puid: "wikidata/29209269",
    name: "Z",
    extensions: &["z"],
    media_types: &["application/x-compress"],
    internal_signatures: &[],
    related_formats: &[],
};
