use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_67173026: FileFormat = FileFormat {
    id: 67_173_026,
    puid: "wikidata/67173026",
    name: "GIMP compressed XJT Image",
    extensions: &["xjt", "xjtbz", "xjtgz"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
