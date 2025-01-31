use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_111342161: FileFormat = FileFormat {
    id: 111_342_161,
    puid: "wikidata/111342161",
    name: "Ad Lib Gold sample",
    extensions: &["smp"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
