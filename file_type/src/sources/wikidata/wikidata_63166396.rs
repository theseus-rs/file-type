use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_63166396: FileFormat = FileFormat {
    id: 63_166_396,
    puid: "wikidata/63166396",
    name: "Microsoft Works Database for Macintosh, version 3",
    extensions: &["wdb"],
    media_types: &["application/vnd.ms-works"],
    internal_signatures: &[],
    related_formats: &[],
};
