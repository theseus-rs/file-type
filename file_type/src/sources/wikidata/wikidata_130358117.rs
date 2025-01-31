use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_130358117: FileFormat = FileFormat {
    id: 130_358_117,
    puid: "wikidata/130358117",
    name: "Mosel source code file",
    extensions: &["mos"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
