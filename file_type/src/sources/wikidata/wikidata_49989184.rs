use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_49989184: FileFormat = FileFormat {
    id: 49_989_184,
    puid: "wikidata/49989184",
    name: "Macro Enabled Microsoft Powerpoint",
    extensions: &["pptm"],
    media_types: &["application/vnd.openxmlformats"],
    internal_signatures: &[],
    related_formats: &[],
};
