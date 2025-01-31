use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_58007215: FileFormat = FileFormat {
    id: 58_007_215,
    puid: "wikidata/58007215",
    name: "Visual Basic File",
    extensions: &["vb", "vb"],
    media_types: &["text/x-vba", "text/x-vbnet"],
    internal_signatures: &[],
    related_formats: &[],
};
