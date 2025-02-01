use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_117324669: FileFormat = FileFormat {
    id: 117_324_669,
    puid: "wikidata/117324669",
    name: "LabWindows/CVI Project file",
    extensions: &["prj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
