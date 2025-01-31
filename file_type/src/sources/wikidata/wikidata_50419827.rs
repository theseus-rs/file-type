use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50419827: FileFormat = FileFormat {
    id: 50_419_827,
    puid: "wikidata/50419827",
    name: "Microsoft PRX File",
    extensions: &["prx"],
    media_types: &["application/octet-stream"],
    internal_signatures: &[],
    related_formats: &[],
};
