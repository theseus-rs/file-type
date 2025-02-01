use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_119163950: FileFormat = FileFormat {
    id: 119_163_950,
    puid: "wikidata/119163950",
    name: "Xstart Settings File",
    extensions: &["xs"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
