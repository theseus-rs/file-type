use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205908: FileFormat = FileFormat {
    id: 28_205_908,
    puid: "wikidata/28205908",
    name: "Digital Imaging and Communications in Medicine file",
    extensions: &["dcm", "dic"],
    media_types: &["application/dicom", "application/dicom"],
    internal_signatures: &[],
    related_formats: &[],
};
