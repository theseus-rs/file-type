use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_49989460: FileFormat = FileFormat {
    id: 49_989_460,
    puid: "wikidata/49989460",
    name: "ActiveX License Package file",
    extensions: &["lpk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
