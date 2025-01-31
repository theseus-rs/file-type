use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28205746: FileFormat = FileFormat {
    id: 28_205_746,
    puid: "wikidata/28205746",
    name: "Microsoft Fax At Work Document",
    extensions: &["awd"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
