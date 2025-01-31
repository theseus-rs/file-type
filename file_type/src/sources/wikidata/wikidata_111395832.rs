use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111395832: FileFormat = FileFormat {
    id: 111_395_832,
    puid: "wikidata/111395832",
    name: "PhotoSuite Slide Show File",
    extensions: &["pzs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
