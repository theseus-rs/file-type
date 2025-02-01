use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28207202: FileFormat = FileFormat {
    id: 28_207_202,
    puid: "wikidata/28207202",
    name: "QuickTime Image Format",
    extensions: &["qif", "qtif"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
