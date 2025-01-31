use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111271825: FileFormat = FileFormat {
    id: 111_271_825,
    puid: "wikidata/111271825",
    name: "Yamaha DX7 voice SysEx dump",
    extensions: &["dx7"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
