use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28551274: FileFormat = FileFormat {
    id: 28_551_274,
    puid: "wikidata/28551274",
    name: "Adobe Arbitrary Map File",
    extensions: &["amp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
