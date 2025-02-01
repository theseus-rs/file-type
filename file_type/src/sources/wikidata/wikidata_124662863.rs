use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_124662863: FileFormat = FileFormat {
    id: 124_662_863,
    puid: "wikidata/124662863",
    name: "Digital Video Broadcasting; Subtitling systems",
    extensions: &["sub"],
    media_types: &["image/vnd.dvb.subtitle"],
    internal_signatures: &[],
    related_formats: &[],
};
