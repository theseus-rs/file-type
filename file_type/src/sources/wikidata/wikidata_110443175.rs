use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_110443175: FileFormat = FileFormat {
    id: 110_443_175,
    puid: "wikidata/110443175",
    name: "Visual Basics MAK File",
    extensions: &["mak"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
