use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28777714: FileFormat = FileFormat {
    id: 28_777_714,
    puid: "wikidata/28777714",
    name: "NII",
    extensions: &["nii"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
