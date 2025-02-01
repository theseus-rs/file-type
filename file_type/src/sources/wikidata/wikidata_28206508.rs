use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206508: FileFormat = FileFormat {
    id: 28_206_508,
    puid: "wikidata/28206508",
    name: "Light Sheet Microscope",
    extensions: &["lsm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
