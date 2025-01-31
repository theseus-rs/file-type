use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205619: FileFormat = FileFormat {
    id: 28_205_619,
    puid: "wikidata/28205619",
    name: "RIPscrip version 2 Hot Icon",
    extensions: &["bmh"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
